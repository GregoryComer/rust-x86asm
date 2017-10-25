use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 217], OperandSize::Dword)
}

fn vcvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 949886603, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 172, 67, 139, 30, 158, 56], OperandSize::Dword)
}

fn vcvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 42, 238], OperandSize::Qword)
}

fn vcvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 838460486, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 42, 132, 177, 70, 228, 249, 49], OperandSize::Qword)
}

fn vcvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 219, 42, 253], OperandSize::Qword)
}

fn vcvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 227, 42, 20, 195], OperandSize::Qword)
}

fn vcvtsi2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 42, 196], OperandSize::Dword)
}

fn vcvtsi2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1013041074, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 42, 164, 199, 178, 199, 97, 60], OperandSize::Dword)
}

fn vcvtsi2sd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 119, 0, 42, 213], OperandSize::Qword)
}

fn vcvtsi2sd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 308969126, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 87, 0, 42, 4, 181, 166, 126, 106, 18], OperandSize::Qword)
}

fn vcvtsi2sd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 207, 56, 42, 219], OperandSize::Qword)
}

fn vcvtsi2sd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1238232632, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 143, 0, 42, 132, 121, 56, 238, 205, 73], OperandSize::Qword)
}

