use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM5)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 213, 63], OperandSize::Dword)
}

fn psrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM2)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 210, 87], OperandSize::Qword)
}

fn psrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 211, 25], OperandSize::Dword)
}

fn psrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 211, 82], OperandSize::Qword)
}

fn psrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 245], OperandSize::Dword)
}

fn psrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 58], OperandSize::Dword)
}

fn psrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 216], OperandSize::Qword)
}

fn psrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1524359430, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 52, 141, 6, 225, 219, 90], OperandSize::Qword)
}

fn psrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 209], OperandSize::Dword)
}

fn psrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 527209215, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 28, 181, 255, 146, 108, 31], OperandSize::Dword)
}

fn psrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 220], OperandSize::Qword)
}

fn psrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 16], OperandSize::Qword)
}

