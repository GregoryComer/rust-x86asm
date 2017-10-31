use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 245], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 1563659248, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 179, 240, 139, 51, 93], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 208], OperandSize::Qword)
}

#[test]
fn cvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 752519856, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 188, 78, 176, 138, 218, 44], OperandSize::Qword)
}

