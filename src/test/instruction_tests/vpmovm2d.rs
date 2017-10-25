use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovm2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 198], OperandSize::Dword)
}

fn vpmovm2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM13)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 56, 236], OperandSize::Qword)
}

fn vpmovm2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM3)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 219], OperandSize::Dword)
}

fn vpmovm2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM4)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 229], OperandSize::Qword)
}

fn vpmovm2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 251], OperandSize::Dword)
}

fn vpmovm2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 56, 204], OperandSize::Qword)
}

