use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn idiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Word)
}

#[test]
fn idiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12930, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 187, 130, 50], OperandSize::Word)
}

#[test]
fn idiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Dword)
}

#[test]
fn idiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1107271288, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 60, 181, 120, 158, 255, 65], OperandSize::Dword)
}

#[test]
fn idiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

#[test]
fn idiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 59], OperandSize::Qword)
}

#[test]
fn idiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Qword)
}

#[test]
fn idiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 56], OperandSize::Qword)
}

#[test]
fn idiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 252], OperandSize::Word)
}

#[test]
fn idiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 61], OperandSize::Word)
}

#[test]
fn idiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 250], OperandSize::Dword)
}

#[test]
fn idiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(EDX, 739174598, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 186, 198, 232, 14, 44], OperandSize::Dword)
}

#[test]
fn idiv_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 253], OperandSize::Qword)
}

#[test]
fn idiv_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 58], OperandSize::Qword)
}

#[test]
fn idiv_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 249], OperandSize::Word)
}

#[test]
fn idiv_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 123, 25], OperandSize::Word)
}

#[test]
fn idiv_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 250], OperandSize::Dword)
}

#[test]
fn idiv_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 60, 152], OperandSize::Dword)
}

#[test]
fn idiv_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 254], OperandSize::Qword)
}

#[test]
fn idiv_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 60, 242], OperandSize::Qword)
}

#[test]
fn idiv_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 251], OperandSize::Qword)
}

#[test]
fn idiv_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(RDX, 881811183, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 186, 239, 94, 143, 52], OperandSize::Qword)
}

