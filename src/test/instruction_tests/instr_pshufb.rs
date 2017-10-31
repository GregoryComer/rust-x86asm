use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 205], OperandSize::Dword)
}

#[test]
fn pshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1765566569, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 44, 93, 105, 104, 60, 105], OperandSize::Dword)
}

#[test]
fn pshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 238], OperandSize::Qword)
}

#[test]
fn pshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 389490357, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 4, 213, 181, 38, 55, 23], OperandSize::Qword)
}

#[test]
fn pshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 205], OperandSize::Dword)
}

#[test]
fn pshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 56], OperandSize::Dword)
}

#[test]
fn pshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 212], OperandSize::Qword)
}

#[test]
fn pshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 20, 119], OperandSize::Qword)
}

