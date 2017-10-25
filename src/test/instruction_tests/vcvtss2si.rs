use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 248], OperandSize::Dword)
}

fn vcvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 30], OperandSize::Dword)
}

fn vcvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 216], OperandSize::Qword)
}

fn vcvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 14], OperandSize::Qword)
}

fn vcvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 215], OperandSize::Qword)
}

fn vcvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1227626302, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 172, 210, 62, 23, 44, 73], OperandSize::Qword)
}

fn vcvtss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 24, 45, 209], OperandSize::Dword)
}

fn vcvtss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1293393118, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 36, 213, 222, 156, 23, 77], OperandSize::Dword)
}

fn vcvtss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 126, 56, 45, 220], OperandSize::Qword)
}

fn vcvtss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1809747749, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 180, 138, 37, 143, 222, 107], OperandSize::Qword)
}

fn vcvtss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 254, 24, 45, 217], OperandSize::Qword)
}

fn vcvtss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 52, 145], OperandSize::Qword)
}

