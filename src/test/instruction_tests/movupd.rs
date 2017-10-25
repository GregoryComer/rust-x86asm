use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 211], OperandSize::Dword)
}

fn movupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 52, 131], OperandSize::Dword)
}

fn movupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 232], OperandSize::Qword)
}

fn movupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 268159240, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 138, 8, 201, 251, 15], OperandSize::Qword)
}

fn movupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 227], OperandSize::Dword)
}

fn movupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 36, 178], OperandSize::Dword)
}

fn movupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 215], OperandSize::Qword)
}

fn movupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 60, 147], OperandSize::Qword)
}

