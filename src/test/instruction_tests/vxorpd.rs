use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vxorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 87, 242], OperandSize::Dword)
}

fn vxorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 87, 48], OperandSize::Dword)
}

fn vxorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 87, 255], OperandSize::Qword)
}

fn vxorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1633310384, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 87, 28, 221, 176, 86, 90, 97], OperandSize::Qword)
}

fn vxorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 87, 224], OperandSize::Dword)
}

fn vxorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 2122333033, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 87, 154, 105, 59, 128, 126], OperandSize::Dword)
}

fn vxorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 87, 252], OperandSize::Qword)
}

fn vxorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 87, 12, 210], OperandSize::Qword)
}

fn vxorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 87, 222], OperandSize::Dword)
}

fn vxorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 87, 6], OperandSize::Dword)
}

fn vxorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 154, 87, 27], OperandSize::Dword)
}

fn vxorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 181, 132, 87, 202], OperandSize::Qword)
}

fn vxorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1793744949, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 173, 130, 87, 156, 81, 53, 96, 234, 106], OperandSize::Qword)
}

fn vxorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 181, 156, 87, 52, 255], OperandSize::Qword)
}

fn vxorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 171, 87, 237], OperandSize::Dword)
}

fn vxorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 87, 52, 153], OperandSize::Dword)
}

fn vxorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 591054117, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 187, 87, 169, 37, 197, 58, 35], OperandSize::Dword)
}

fn vxorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 157, 172, 87, 246], OperandSize::Qword)
}

fn vxorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 174, 87, 34], OperandSize::Qword)
}

fn vxorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 2071771584, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 213, 177, 87, 44, 149, 192, 185, 124, 123], OperandSize::Qword)
}

fn vxorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 87, 216], OperandSize::Dword)
}

fn vxorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 87, 55], OperandSize::Dword)
}

fn vxorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 223, 87, 3], OperandSize::Dword)
}

fn vxorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 165, 203, 87, 213], OperandSize::Qword)
}

fn vxorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 493082594, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 181, 199, 87, 28, 141, 226, 215, 99, 29], OperandSize::Qword)
}

fn vxorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 173, 209, 87, 20, 129], OperandSize::Qword)
}

