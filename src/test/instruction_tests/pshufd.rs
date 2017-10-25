use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 197, 94], OperandSize::Dword)
}

fn pshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1779013081, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 148, 176, 217, 149, 9, 106, 23], OperandSize::Dword)
}

fn pshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 205, 82], OperandSize::Qword)
}

fn pshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 26, 110], OperandSize::Qword)
}

