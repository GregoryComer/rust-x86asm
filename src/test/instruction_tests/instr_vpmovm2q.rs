use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 56, 203], OperandSize::Dword)
}

#[test]
fn vpmovm2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM31)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 8, 56, 253], OperandSize::Qword)
}

#[test]
fn vpmovm2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 56, 246], OperandSize::Dword)
}

#[test]
fn vpmovm2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM30)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 40, 56, 244], OperandSize::Qword)
}

#[test]
fn vpmovm2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 56, 227], OperandSize::Dword)
}

#[test]
fn vpmovm2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 254, 72, 56, 213], OperandSize::Qword)
}

