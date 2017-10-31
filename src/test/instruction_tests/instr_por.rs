use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn por_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 242], OperandSize::Dword)
}

#[test]
fn por_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 356157149, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 20, 93, 221, 134, 58, 21], OperandSize::Dword)
}

#[test]
fn por_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 255], OperandSize::Qword)
}

#[test]
fn por_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 62], OperandSize::Qword)
}

#[test]
fn por_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 247], OperandSize::Dword)
}

#[test]
fn por_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1879129597, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 163, 253, 61, 1, 112], OperandSize::Dword)
}

#[test]
fn por_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 199], OperandSize::Qword)
}

#[test]
fn por_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 36, 218], OperandSize::Qword)
}

