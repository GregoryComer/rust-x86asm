use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovq2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 218], OperandSize::Dword)
}

fn vpmovq2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 254, 8, 57, 248], OperandSize::Qword)
}

fn vpmovq2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 57, 215], OperandSize::Dword)
}

fn vpmovq2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 254, 40, 57, 255], OperandSize::Qword)
}

fn vpmovq2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 57, 235], OperandSize::Dword)
}

fn vpmovq2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 254, 72, 57, 242], OperandSize::Qword)
}

