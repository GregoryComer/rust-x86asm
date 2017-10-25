use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 190, 236], OperandSize::Dword)
}

fn vfnmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 190, 57], OperandSize::Dword)
}

fn vfnmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 190, 250], OperandSize::Qword)
}

fn vfnmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 2070735141, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 190, 180, 79, 37, 233, 108, 123], OperandSize::Qword)
}

fn vfnmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 190, 235], OperandSize::Dword)
}

fn vfnmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 190, 34], OperandSize::Dword)
}

fn vfnmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 190, 210], OperandSize::Qword)
}

fn vfnmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1177981167, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 190, 148, 184, 239, 144, 54, 70], OperandSize::Qword)
}

fn vfnmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 190, 249], OperandSize::Dword)
}

fn vfnmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 190, 20, 198], OperandSize::Dword)
}

fn vfnmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1812040415, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 154, 190, 188, 73, 223, 138, 1, 108], OperandSize::Dword)
}

fn vfnmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 141, 133, 190, 246], OperandSize::Qword)
}

fn vfnmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 190, 48], OperandSize::Qword)
}

fn vfnmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 205, 151, 190, 44, 64], OperandSize::Qword)
}

fn vfnmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 190, 220], OperandSize::Dword)
}

fn vfnmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1987518017, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 190, 148, 143, 65, 30, 119, 118], OperandSize::Dword)
}

fn vfnmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 185, 190, 41], OperandSize::Dword)
}

fn vfnmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 149, 170, 190, 207], OperandSize::Qword)
}

fn vfnmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1374279683, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 197, 166, 190, 36, 93, 3, 216, 233, 81], OperandSize::Qword)
}

fn vfnmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1806302099, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 190, 28, 213, 147, 251, 169, 107], OperandSize::Qword)
}

fn vfnmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 190, 190, 228], OperandSize::Dword)
}

fn vfnmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 330925131, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 190, 166, 75, 132, 185, 19], OperandSize::Dword)
}

fn vfnmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 190, 60, 208], OperandSize::Dword)
}

fn vfnmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 205, 189, 190, 249], OperandSize::Qword)
}

fn vfnmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 141, 201, 190, 42], OperandSize::Qword)
}

fn vfnmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 744650457, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 213, 213, 190, 180, 176, 217, 118, 98, 44], OperandSize::Qword)
}

