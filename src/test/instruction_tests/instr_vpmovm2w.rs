use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM6)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 246], OperandSize::Dword)
}

#[test]
fn vpmovm2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM22)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 8, 40, 246], OperandSize::Qword)
}

#[test]
fn vpmovm2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM5)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 40, 239], OperandSize::Dword)
}

#[test]
fn vpmovm2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM9)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 254, 40, 40, 207], OperandSize::Qword)
}

#[test]
fn vpmovm2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 218], OperandSize::Dword)
}

#[test]
fn vpmovm2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 214], OperandSize::Qword)
}

