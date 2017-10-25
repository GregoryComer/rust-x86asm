use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 220, 202], OperandSize::Dword)
}

fn vpaddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 220, 26], OperandSize::Dword)
}

fn vpaddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 220, 243], OperandSize::Qword)
}

fn vpaddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 220, 28, 202], OperandSize::Qword)
}

fn vpaddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 220, 221], OperandSize::Dword)
}

fn vpaddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 197092609, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 220, 144, 1, 101, 191, 11], OperandSize::Dword)
}

fn vpaddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 220, 205], OperandSize::Qword)
}

fn vpaddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 220, 9], OperandSize::Qword)
}

fn vpaddusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 220, 247], OperandSize::Dword)
}

fn vpaddusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 211695574, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 220, 28, 77, 214, 55, 158, 12], OperandSize::Dword)
}

fn vpaddusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 125, 137, 220, 218], OperandSize::Qword)
}

fn vpaddusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 13, 142, 220, 44, 208], OperandSize::Qword)
}

fn vpaddusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 220, 201], OperandSize::Dword)
}

fn vpaddusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 220, 4, 150], OperandSize::Dword)
}

fn vpaddusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 45, 164, 220, 215], OperandSize::Qword)
}

fn vpaddusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 45, 164, 220, 20, 129], OperandSize::Qword)
}

fn vpaddusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 220, 223], OperandSize::Dword)
}

fn vpaddusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 220, 4, 129], OperandSize::Dword)
}

fn vpaddusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 101, 205, 220, 207], OperandSize::Qword)
}

fn vpaddusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 45, 207, 220, 33], OperandSize::Qword)
}

