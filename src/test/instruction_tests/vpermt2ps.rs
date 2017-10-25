use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermt2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 127, 229], OperandSize::Dword)
}

fn vpermt2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 359189203, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 127, 52, 77, 211, 202, 104, 21], OperandSize::Dword)
}

fn vpermt2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 127, 12, 138], OperandSize::Dword)
}

fn vpermt2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 5, 131, 127, 249], OperandSize::Qword)
}

fn vpermt2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 2011169681, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 45, 129, 127, 164, 66, 145, 3, 224, 119], OperandSize::Qword)
}

fn vpermt2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 109136662, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 109, 157, 127, 28, 157, 22, 75, 129, 6], OperandSize::Qword)
}

fn vpermt2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 127, 201], OperandSize::Dword)
}

fn vpermt2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1270087396, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 127, 140, 240, 228, 254, 179, 75], OperandSize::Dword)
}

fn vpermt2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 127, 26], OperandSize::Dword)
}

fn vpermt2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 53, 170, 127, 218], OperandSize::Qword)
}

fn vpermt2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 29, 161, 127, 52, 71], OperandSize::Qword)
}

fn vpermt2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 183, 127, 36, 90], OperandSize::Qword)
}

fn vpermt2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 203, 127, 250], OperandSize::Dword)
}

fn vpermt2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 684795722, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 127, 161, 74, 39, 209, 40], OperandSize::Dword)
}

fn vpermt2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 902717091, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 220, 127, 52, 157, 163, 94, 206, 53], OperandSize::Dword)
}

fn vpermt2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 21, 202, 127, 241], OperandSize::Qword)
}

fn vpermt2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 93, 197, 127, 34], OperandSize::Qword)
}

fn vpermt2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDI, 728380936, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 219, 127, 151, 8, 54, 106, 43], OperandSize::Qword)
}

