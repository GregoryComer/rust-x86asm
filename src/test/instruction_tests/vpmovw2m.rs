use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovw2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 41, 223], OperandSize::Dword)
}

fn vpmovw2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 41, 210], OperandSize::Qword)
}

fn vpmovw2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 41, 246], OperandSize::Dword)
}

fn vpmovw2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 254, 40, 41, 219], OperandSize::Qword)
}

fn vpmovw2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 41, 249], OperandSize::Dword)
}

fn vpmovw2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 254, 72, 41, 233], OperandSize::Qword)
}

