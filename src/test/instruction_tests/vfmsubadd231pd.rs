use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 220], OperandSize::Dword)
}

fn vfmsubadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 850512100, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 152, 228, 200, 177, 50], OperandSize::Dword)
}

fn vfmsubadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 183, 236], OperandSize::Qword)
}

fn vfmsubadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 183, 4, 192], OperandSize::Qword)
}

fn vfmsubadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 183, 241], OperandSize::Dword)
}

fn vfmsubadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 183, 28, 251], OperandSize::Dword)
}

fn vfmsubadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 183, 251], OperandSize::Qword)
}

fn vfmsubadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2130291831, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 183, 28, 85, 119, 172, 249, 126], OperandSize::Qword)
}

fn vfmsubadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 183, 211], OperandSize::Dword)
}

fn vfmsubadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 183, 36, 147], OperandSize::Dword)
}

fn vfmsubadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 1274354374, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 183, 155, 198, 26, 245, 75], OperandSize::Dword)
}

fn vfmsubadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 157, 132, 183, 213], OperandSize::Qword)
}

fn vfmsubadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 137, 183, 28, 64], OperandSize::Qword)
}

fn vfmsubadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 158, 183, 38], OperandSize::Qword)
}

fn vfmsubadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 183, 236], OperandSize::Dword)
}

fn vfmsubadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 290802606, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 183, 177, 174, 75, 85, 17], OperandSize::Dword)
}

fn vfmsubadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1625810988, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 191, 183, 161, 44, 232, 231, 96], OperandSize::Dword)
}

fn vfmsubadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 167, 183, 249], OperandSize::Qword)
}

fn vfmsubadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1130217280, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 165, 161, 183, 188, 210, 64, 191, 93, 67], OperandSize::Qword)
}

fn vfmsubadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2127194374, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 141, 178, 183, 4, 141, 6, 105, 202, 126], OperandSize::Qword)
}

fn vfmsubadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 223, 183, 218], OperandSize::Dword)
}

fn vfmsubadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 2031267646, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 183, 44, 189, 62, 175, 18, 121], OperandSize::Dword)
}

fn vfmsubadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1411659700, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 220, 183, 28, 149, 180, 55, 36, 84], OperandSize::Dword)
}

fn vfmsubadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 229, 180, 183, 247], OperandSize::Qword)
}

fn vfmsubadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 966262257, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 207, 183, 156, 194, 241, 253, 151, 57], OperandSize::Qword)
}

fn vfmsubadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 173, 219, 183, 44, 152], OperandSize::Qword)
}

