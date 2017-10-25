use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 182, 208], OperandSize::Dword)
}

fn vfmaddsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1999563503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 182, 143, 239, 234, 46, 119], OperandSize::Dword)
}

fn vfmaddsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 182, 228], OperandSize::Qword)
}

fn vfmaddsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 182, 52, 88], OperandSize::Qword)
}

fn vfmaddsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 182, 245], OperandSize::Dword)
}

fn vfmaddsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 2071380885, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 182, 4, 149, 149, 195, 118, 123], OperandSize::Dword)
}

fn vfmaddsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 182, 213], OperandSize::Qword)
}

fn vfmaddsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1009773483, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 182, 36, 149, 171, 235, 47, 60], OperandSize::Qword)
}

fn vfmaddsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 182, 221], OperandSize::Dword)
}

fn vfmaddsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 182, 12, 86], OperandSize::Dword)
}

fn vfmaddsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1942463390, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 158, 182, 52, 141, 158, 163, 199, 115], OperandSize::Dword)
}

fn vfmaddsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 237, 143, 182, 217], OperandSize::Qword)
}

fn vfmaddsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 165, 142, 182, 62], OperandSize::Qword)
}

fn vfmaddsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 640309694, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 151, 182, 28, 181, 190, 89, 42, 38], OperandSize::Qword)
}

fn vfmaddsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 182, 242], OperandSize::Dword)
}

fn vfmaddsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 992147886, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 182, 172, 176, 174, 249, 34, 59], OperandSize::Dword)
}

fn vfmaddsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1673422858, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 191, 182, 20, 181, 10, 104, 190, 99], OperandSize::Dword)
}

fn vfmaddsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 245, 165, 182, 248], OperandSize::Qword)
}

fn vfmaddsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1936899291, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 182, 44, 117, 219, 188, 114, 115], OperandSize::Qword)
}

fn vfmaddsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RSI, 1597284314, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 173, 179, 182, 190, 218, 159, 52, 95], OperandSize::Qword)
}

fn vfmaddsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 250, 182, 211], OperandSize::Dword)
}

fn vfmaddsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ESI, 1719951809, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 182, 150, 193, 97, 132, 102], OperandSize::Dword)
}

fn vfmaddsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 223, 182, 36, 183], OperandSize::Dword)
}

fn vfmaddsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 173, 243, 182, 228], OperandSize::Qword)
}

fn vfmaddsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1053334485, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 245, 198, 182, 36, 189, 213, 155, 200, 62], OperandSize::Qword)
}

fn vfmaddsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 189, 210, 182, 40], OperandSize::Qword)
}

