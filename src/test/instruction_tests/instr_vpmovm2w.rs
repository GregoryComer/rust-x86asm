use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM6)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 244], OperandSize::Dword)
}

#[test]
fn vpmovm2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM19)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 8, 40, 220], OperandSize::Qword)
}

#[test]
fn vpmovm2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM0)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 40, 198], OperandSize::Dword)
}

#[test]
fn vpmovm2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM19)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 40, 40, 220], OperandSize::Qword)
}

#[test]
fn vpmovm2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 236], OperandSize::Dword)
}

#[test]
fn vpmovm2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 206], OperandSize::Qword)
}

