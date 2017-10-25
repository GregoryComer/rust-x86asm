use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 174, 211], OperandSize::Dword)
}

fn vfnmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1882510576, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 174, 140, 187, 240, 212, 52, 112], OperandSize::Dword)
}

fn vfnmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 226], OperandSize::Qword)
}

fn vfnmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 304177994, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 52, 85, 74, 99, 33, 18], OperandSize::Qword)
}

fn vfnmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 174, 224], OperandSize::Dword)
}

fn vfnmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 174, 41], OperandSize::Dword)
}

fn vfnmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 174, 227], OperandSize::Qword)
}

fn vfnmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1703705186, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 174, 172, 71, 98, 122, 140, 101], OperandSize::Qword)
}

fn vfnmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 174, 202], OperandSize::Dword)
}

fn vfnmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1324929778, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 174, 183, 242, 210, 248, 78], OperandSize::Dword)
}

fn vfnmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 328983136, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 174, 176, 96, 226, 155, 19], OperandSize::Dword)
}

fn vfnmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 101, 132, 174, 252], OperandSize::Qword)
}

fn vfnmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 918187778, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 130, 174, 188, 71, 2, 111, 186, 54], OperandSize::Qword)
}

fn vfnmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 150, 174, 18], OperandSize::Qword)
}

fn vfnmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 174, 244], OperandSize::Dword)
}

fn vfnmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 174, 52, 194], OperandSize::Dword)
}

fn vfnmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 618330440, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 185, 174, 60, 221, 72, 249, 218, 36], OperandSize::Dword)
}

fn vfnmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 5, 169, 174, 194], OperandSize::Qword)
}

fn vfnmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 163, 174, 52, 177], OperandSize::Qword)
}

fn vfnmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 776597066, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 21, 188, 174, 36, 181, 74, 238, 73, 46], OperandSize::Qword)
}

fn vfnmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 223, 174, 202], OperandSize::Dword)
}

fn vfnmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 95429592, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 174, 20, 125, 216, 35, 176, 5], OperandSize::Dword)
}

fn vfnmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1020673878, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 174, 20, 245, 86, 63, 214, 60], OperandSize::Dword)
}

fn vfnmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 21, 150, 174, 243], OperandSize::Qword)
}

fn vfnmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1248228010, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 174, 164, 184, 170, 114, 102, 74], OperandSize::Qword)
}

fn vfnmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 994051166, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 212, 174, 132, 202, 94, 4, 64, 59], OperandSize::Qword)
}

