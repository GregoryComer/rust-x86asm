use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovd2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 57, 252], OperandSize::Dword)
}

fn vpmovd2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 8, 57, 216], OperandSize::Qword)
}

fn vpmovd2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 57, 220], OperandSize::Dword)
}

fn vpmovd2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 40, 57, 232], OperandSize::Qword)
}

fn vpmovd2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 57, 236], OperandSize::Dword)
}

fn vpmovd2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 72, 57, 238], OperandSize::Qword)
}

