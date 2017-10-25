use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 188, 252], OperandSize::Dword)
}

fn vfnmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1292797255, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 188, 44, 125, 71, 133, 14, 77], OperandSize::Dword)
}

fn vfnmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 188, 199], OperandSize::Qword)
}

fn vfnmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 188, 44, 134], OperandSize::Qword)
}

fn vfnmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 188, 240], OperandSize::Dword)
}

fn vfnmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1589736623, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 188, 167, 175, 116, 193, 94], OperandSize::Dword)
}

fn vfnmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 188, 213], OperandSize::Qword)
}

fn vfnmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RSI, 465341040, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 188, 182, 112, 138, 188, 27], OperandSize::Qword)
}

fn vfnmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 188, 208], OperandSize::Dword)
}

fn vfnmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1001739591, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 188, 60, 141, 71, 85, 181, 59], OperandSize::Dword)
}

fn vfnmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2023203850, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 156, 188, 20, 181, 10, 164, 151, 120], OperandSize::Dword)
}

fn vfnmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 149, 142, 188, 202], OperandSize::Qword)
}

fn vfnmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 197, 133, 188, 6], OperandSize::Qword)
}

fn vfnmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 145, 188, 51], OperandSize::Qword)
}

fn vfnmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 188, 238], OperandSize::Dword)
}

fn vfnmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 398615298, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 188, 20, 149, 2, 99, 194, 23], OperandSize::Dword)
}

fn vfnmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1880928531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 185, 188, 150, 19, 177, 28, 112], OperandSize::Dword)
}

fn vfnmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 173, 161, 188, 232], OperandSize::Qword)
}

fn vfnmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 165, 171, 188, 44, 190], OperandSize::Qword)
}

fn vfnmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1444264456, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 173, 185, 188, 132, 198, 8, 186, 21, 86], OperandSize::Qword)
}

fn vfnmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 157, 188, 204], OperandSize::Dword)
}

fn vfnmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 188, 40], OperandSize::Dword)
}

fn vfnmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDX, 86950591, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 188, 138, 191, 194, 46, 5], OperandSize::Dword)
}

fn vfnmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 229, 255, 188, 217], OperandSize::Qword)
}

fn vfnmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 188, 8], OperandSize::Qword)
}

fn vfnmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 213, 188, 36, 136], OperandSize::Qword)
}

