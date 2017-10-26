use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn idiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Word)
}

#[test]
fn idiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 28666, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 186, 250, 111], OperandSize::Word)
}

#[test]
fn idiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 250], OperandSize::Dword)
}

#[test]
fn idiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 56], OperandSize::Dword)
}

#[test]
fn idiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

#[test]
fn idiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 62], OperandSize::Qword)
}

#[test]
fn idiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

#[test]
fn idiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 472418273, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 188, 112, 225, 135, 40, 28], OperandSize::Qword)
}

#[test]
fn idiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 253], OperandSize::Word)
}

#[test]
fn idiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 59], OperandSize::Word)
}

#[test]
fn idiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 249], OperandSize::Dword)
}

#[test]
fn idiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 142], OperandSize::Dword)
}

#[test]
fn idiv_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 250], OperandSize::Qword)
}

#[test]
fn idiv_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1518740250, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 188, 129, 26, 35, 134, 90], OperandSize::Qword)
}

#[test]
fn idiv_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 249], OperandSize::Word)
}

#[test]
fn idiv_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 10880, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 185, 128, 42], OperandSize::Word)
}

#[test]
fn idiv_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 250], OperandSize::Dword)
}

#[test]
fn idiv_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 56], OperandSize::Dword)
}

#[test]
fn idiv_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 250], OperandSize::Qword)
}

#[test]
fn idiv_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 59], OperandSize::Qword)
}

#[test]
fn idiv_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 253], OperandSize::Qword)
}

#[test]
fn idiv_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 60, 78], OperandSize::Qword)
}

