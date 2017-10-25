use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovb2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 237], OperandSize::Dword)
}

fn vpmovb2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 8, 41, 206], OperandSize::Qword)
}

fn vpmovb2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 41, 255], OperandSize::Dword)
}

fn vpmovb2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 40, 41, 215], OperandSize::Qword)
}

fn vpmovb2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 218], OperandSize::Dword)
}

fn vpmovb2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 72, 41, 212], OperandSize::Qword)
}

