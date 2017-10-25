use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovm2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM4)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 56, 225], OperandSize::Dword)
}

fn vpmovm2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(XMM8)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 254, 8, 56, 198], OperandSize::Qword)
}

fn vpmovm2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 56, 233], OperandSize::Dword)
}

fn vpmovm2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(YMM18)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 40, 56, 214], OperandSize::Qword)
}

fn vpmovm2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 56, 231], OperandSize::Dword)
}

fn vpmovm2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2Q, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 72, 56, 252], OperandSize::Qword)
}

