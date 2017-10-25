use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 232, 195], OperandSize::Dword)
}

fn vpsubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1035721555, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 232, 60, 221, 83, 219, 187, 61], OperandSize::Dword)
}

fn vpsubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 232, 212], OperandSize::Qword)
}

fn vpsubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 232, 28, 151], OperandSize::Qword)
}

fn vpsubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 223], OperandSize::Dword)
}

fn vpsubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 46767318, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 232, 143, 214, 156, 201, 2], OperandSize::Dword)
}

fn vpsubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 210], OperandSize::Qword)
}

fn vpsubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1911083314, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 232, 140, 195, 50, 209, 232, 113], OperandSize::Qword)
}

fn vpsubsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 232, 206], OperandSize::Dword)
}

fn vpsubsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 710202693, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 232, 156, 71, 69, 213, 84, 42], OperandSize::Dword)
}

fn vpsubsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 29, 129, 232, 227], OperandSize::Qword)
}

fn vpsubsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 69, 134, 232, 52, 146], OperandSize::Qword)
}

fn vpsubsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 232, 246], OperandSize::Dword)
}

fn vpsubsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1056473114, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 232, 52, 117, 26, 128, 248, 62], OperandSize::Dword)
}

fn vpsubsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 13, 162, 232, 223], OperandSize::Qword)
}

fn vpsubsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 100149581, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 109, 166, 232, 52, 205, 77, 41, 248, 5], OperandSize::Qword)
}

fn vpsubsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 232, 252], OperandSize::Dword)
}

fn vpsubsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 232, 4, 248], OperandSize::Dword)
}

fn vpsubsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 77, 199, 232, 199], OperandSize::Qword)
}

fn vpsubsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 619956163, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 196, 232, 180, 199, 195, 199, 243, 36], OperandSize::Qword)
}

