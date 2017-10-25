use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 210], OperandSize::Dword)
}

fn aesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 939847747, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 132, 128, 67, 240, 4, 56], OperandSize::Dword)
}

fn aesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 253], OperandSize::Qword)
}

fn aesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 478839049, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 159, 9, 129, 138, 28], OperandSize::Qword)
}

