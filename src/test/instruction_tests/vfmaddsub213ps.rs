use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 166, 255], OperandSize::Dword)
}

fn vfmaddsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 166, 28, 67], OperandSize::Dword)
}

fn vfmaddsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 166, 226], OperandSize::Qword)
}

fn vfmaddsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 166, 20, 210], OperandSize::Qword)
}

fn vfmaddsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 166, 235], OperandSize::Dword)
}

fn vfmaddsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 166, 12, 120], OperandSize::Dword)
}

fn vfmaddsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 166, 251], OperandSize::Qword)
}

fn vfmaddsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 166, 63], OperandSize::Qword)
}

fn vfmaddsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 166, 196], OperandSize::Dword)
}

fn vfmaddsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 300828003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 166, 52, 133, 99, 69, 238, 17], OperandSize::Dword)
}

fn vfmaddsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 159, 166, 52, 79], OperandSize::Dword)
}

fn vfmaddsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 142, 166, 230], OperandSize::Qword)
}

fn vfmaddsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 921723653, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 93, 139, 166, 153, 5, 99, 240, 54], OperandSize::Qword)
}

fn vfmaddsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1613619070, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 13, 146, 166, 132, 73, 126, 223, 45, 96], OperandSize::Qword)
}

fn vfmaddsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 171, 166, 206], OperandSize::Dword)
}

fn vfmaddsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 166, 63], OperandSize::Dword)
}

fn vfmaddsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1060845969, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 185, 166, 36, 117, 145, 57, 59, 63], OperandSize::Dword)
}

fn vfmaddsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 117, 167, 166, 251], OperandSize::Qword)
}

fn vfmaddsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RCX, 1156191814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 61, 165, 166, 129, 70, 22, 234, 68], OperandSize::Qword)
}

fn vfmaddsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 101, 178, 166, 34], OperandSize::Qword)
}

fn vfmaddsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 154, 166, 225], OperandSize::Dword)
}

fn vfmaddsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 198146833, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 207, 166, 136, 17, 123, 207, 11], OperandSize::Dword)
}

fn vfmaddsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 1921046755, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 220, 166, 131, 227, 216, 128, 114], OperandSize::Dword)
}

fn vfmaddsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 5, 213, 166, 193], OperandSize::Qword)
}

fn vfmaddsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1299732925, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 117, 197, 166, 12, 181, 189, 89, 120, 77], OperandSize::Qword)
}

fn vfmaddsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 53, 213, 166, 60, 81], OperandSize::Qword)
}

