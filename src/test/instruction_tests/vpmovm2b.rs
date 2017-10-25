use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovm2b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 40, 196], OperandSize::Dword)
}

fn vpmovm2b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM17)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 40, 206], OperandSize::Qword)
}

fn vpmovm2b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM0)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 40, 197], OperandSize::Dword)
}

fn vpmovm2b_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM23)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 40, 251], OperandSize::Qword)
}

fn vpmovm2b_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 40, 252], OperandSize::Dword)
}

fn vpmovm2b_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 40, 230], OperandSize::Qword)
}

