use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 235], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 47], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 242], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 827348909, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 44, 85, 173, 87, 80, 49], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 234], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 4, 121], OperandSize::Qword)
}

