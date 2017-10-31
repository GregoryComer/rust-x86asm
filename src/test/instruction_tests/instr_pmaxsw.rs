use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 195], OperandSize::Dword)
}

#[test]
fn pmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 2102946738, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 180, 90, 178, 107, 88, 125], OperandSize::Dword)
}

#[test]
fn pmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 240], OperandSize::Qword)
}

#[test]
fn pmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RSI, 1273738326, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 150, 86, 180, 235, 75], OperandSize::Qword)
}

#[test]
fn pmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 255], OperandSize::Dword)
}

#[test]
fn pmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1377805139, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 164, 195, 83, 163, 31, 82], OperandSize::Dword)
}

#[test]
fn pmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 210], OperandSize::Qword)
}

#[test]
fn pmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1840883151, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 172, 179, 207, 165, 185, 109], OperandSize::Qword)
}

