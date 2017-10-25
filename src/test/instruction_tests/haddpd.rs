use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn haddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 220], OperandSize::Dword)
}

fn haddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1226455494, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 20, 157, 198, 57, 26, 73], OperandSize::Dword)
}

fn haddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 222], OperandSize::Qword)
}

fn haddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 347959041, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 20, 213, 1, 111, 189, 20], OperandSize::Qword)
}

