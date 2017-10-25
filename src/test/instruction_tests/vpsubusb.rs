use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 216, 208], OperandSize::Dword)
}

fn vpsubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 216, 3], OperandSize::Dword)
}

fn vpsubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 216, 251], OperandSize::Qword)
}

fn vpsubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 715345256, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 216, 180, 242, 104, 77, 163, 42], OperandSize::Qword)
}

fn vpsubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 216, 193], OperandSize::Dword)
}

fn vpsubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 216, 19], OperandSize::Dword)
}

fn vpsubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 216, 238], OperandSize::Qword)
}

fn vpsubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1318634075, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 216, 36, 117, 91, 194, 152, 78], OperandSize::Qword)
}

fn vpsubusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 216, 212], OperandSize::Dword)
}

fn vpsubusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 139, 216, 44, 176], OperandSize::Dword)
}

fn vpsubusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 53, 143, 216, 242], OperandSize::Qword)
}

fn vpsubusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 487116962, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 53, 143, 216, 132, 152, 162, 208, 8, 29], OperandSize::Qword)
}

fn vpsubusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 216, 225], OperandSize::Dword)
}

fn vpsubusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 216, 56], OperandSize::Dword)
}

fn vpsubusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 5, 164, 216, 248], OperandSize::Qword)
}

fn vpsubusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 93, 175, 216, 46], OperandSize::Qword)
}

fn vpsubusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 216, 223], OperandSize::Dword)
}

fn vpsubusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 216, 52, 139], OperandSize::Dword)
}

fn vpsubusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 77, 194, 216, 200], OperandSize::Qword)
}

fn vpsubusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 197, 216, 55], OperandSize::Qword)
}

