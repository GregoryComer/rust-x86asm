use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 228], OperandSize::Dword)
}

#[test]
fn pshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1009906426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 187, 250, 242, 49, 60], OperandSize::Dword)
}

#[test]
fn pshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 219], OperandSize::Qword)
}

#[test]
fn pshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 58], OperandSize::Qword)
}

#[test]
fn pshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 214], OperandSize::Dword)
}

#[test]
fn pshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 18], OperandSize::Dword)
}

#[test]
fn pshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 213], OperandSize::Qword)
}

#[test]
fn pshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 44, 218], OperandSize::Qword)
}

