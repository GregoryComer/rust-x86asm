use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 229], OperandSize::Dword)
}

#[test]
fn psubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 8], OperandSize::Dword)
}

#[test]
fn psubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 193], OperandSize::Qword)
}

#[test]
fn psubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDI, 637341059, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 151, 131, 13, 253, 37], OperandSize::Qword)
}

#[test]
fn psubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 208], OperandSize::Dword)
}

#[test]
fn psubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 830381291, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 4, 125, 235, 156, 126, 49], OperandSize::Dword)
}

#[test]
fn psubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 215], OperandSize::Qword)
}

#[test]
fn psubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 4, 191], OperandSize::Qword)
}

