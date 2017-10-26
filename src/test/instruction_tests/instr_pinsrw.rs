use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM3)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 220, 7], OperandSize::Dword)
}

#[test]
fn pinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(EBX, 1657474086, Some(OperandSize::Word), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 139, 38, 12, 203, 98, 121], OperandSize::Dword)
}

#[test]
fn pinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM1)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 202, 17], OperandSize::Qword)
}

#[test]
fn pinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Word), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 52, 190, 82], OperandSize::Qword)
}

#[test]
fn pinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 229, 42], OperandSize::Dword)
}

#[test]
fn pinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1844815527, Some(OperandSize::Word), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 60, 245, 167, 166, 245, 109, 101], OperandSize::Dword)
}

#[test]
fn pinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 246, 9], OperandSize::Qword)
}

#[test]
fn pinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Word), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 20, 131, 68], OperandSize::Qword)
}

