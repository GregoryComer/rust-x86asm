use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 56, 198], OperandSize::Dword)
}

#[test]
fn vpmovm2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM23)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 8, 56, 250], OperandSize::Qword)
}

#[test]
fn vpmovm2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 56, 203], OperandSize::Dword)
}

#[test]
fn vpmovm2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM26)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 40, 56, 211], OperandSize::Qword)
}

#[test]
fn vpmovm2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 56, 223], OperandSize::Dword)
}

#[test]
fn vpmovm2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 72, 56, 242], OperandSize::Qword)
}

