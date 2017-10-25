use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha256msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 210], OperandSize::Dword)
}

fn sha256msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 744437223, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 148, 73, 231, 53, 95, 44], OperandSize::Dword)
}

fn sha256msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 194], OperandSize::Qword)
}

fn sha256msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256MSG2, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 490667473, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 205, 172, 71, 209, 253, 62, 29], OperandSize::Qword)
}

