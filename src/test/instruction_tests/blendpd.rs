use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 231, 94], OperandSize::Dword)
}

fn blendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 44, 131, 83], OperandSize::Dword)
}

fn blendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 251, 74], OperandSize::Qword)
}

fn blendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1272072500, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 156, 120, 52, 73, 210, 75, 55], OperandSize::Qword)
}

