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
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 241, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 187, 241, 0], OperandSize::Word)
}

#[test]
fn idiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Dword)
}

#[test]
fn idiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1384998242, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 188, 216, 98, 101, 141, 82], OperandSize::Dword)
}

#[test]
fn idiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Qword)
}

#[test]
fn idiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 58], OperandSize::Qword)
}

#[test]
fn idiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

#[test]
fn idiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 62], OperandSize::Qword)
}

#[test]
fn idiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 255], OperandSize::Word)
}

#[test]
fn idiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 125, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 123, 125], OperandSize::Word)
}

#[test]
fn idiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 252], OperandSize::Dword)
}

#[test]
fn idiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 58], OperandSize::Dword)
}

#[test]
fn idiv_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 255], OperandSize::Qword)
}

#[test]
fn idiv_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 203], OperandSize::Qword)
}

#[test]
fn idiv_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 251], OperandSize::Word)
}

#[test]
fn idiv_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(DI, 131, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 189, 131, 0], OperandSize::Word)
}

#[test]
fn idiv_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 255], OperandSize::Dword)
}

#[test]
fn idiv_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1335570469, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 60, 117, 37, 48, 155, 79], OperandSize::Dword)
}

#[test]
fn idiv_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 252], OperandSize::Qword)
}

#[test]
fn idiv_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 56], OperandSize::Qword)
}

#[test]
fn idiv_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 251], OperandSize::Qword)
}

#[test]
fn idiv_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 231400624, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 188, 210, 176, 228, 202, 13], OperandSize::Qword)
}

