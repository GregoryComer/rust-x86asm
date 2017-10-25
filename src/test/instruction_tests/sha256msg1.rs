use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha256msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 237], OperandSize::Dword)
}

fn sha256msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 28, 72], OperandSize::Dword)
}

fn sha256msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 204], OperandSize::Qword)
}

fn sha256msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG1, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 734943748, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 204, 60, 149, 4, 90, 206, 43], OperandSize::Qword)
}

