use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovm2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 194], OperandSize::Dword)
}

fn vpmovm2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 194], OperandSize::Qword)
}

fn vpmovm2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM0)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 40, 195], OperandSize::Dword)
}

fn vpmovm2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM27)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 40, 40, 219], OperandSize::Qword)
}

fn vpmovm2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 250], OperandSize::Dword)
}

fn vpmovm2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 254, 72, 40, 226], OperandSize::Qword)
}

