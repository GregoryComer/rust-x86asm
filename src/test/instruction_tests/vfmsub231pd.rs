use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 186, 242], OperandSize::Dword)
}

fn vfmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 186, 60, 143], OperandSize::Dword)
}

fn vfmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 186, 225], OperandSize::Qword)
}

fn vfmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RAX, 42715254, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 186, 176, 118, 200, 139, 2], OperandSize::Qword)
}

fn vfmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 186, 203], OperandSize::Dword)
}

fn vfmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 186, 63], OperandSize::Dword)
}

fn vfmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 186, 234], OperandSize::Qword)
}

fn vfmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 186, 60, 126], OperandSize::Qword)
}

fn vfmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 186, 198], OperandSize::Dword)
}

fn vfmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 860622024, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 186, 156, 113, 200, 12, 76, 51], OperandSize::Dword)
}

fn vfmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1454351137, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 158, 186, 148, 146, 33, 163, 175, 86], OperandSize::Dword)
}

fn vfmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 173, 141, 186, 229], OperandSize::Qword)
}

fn vfmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RBX, 902939374, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 130, 186, 171, 238, 194, 209, 53], OperandSize::Qword)
}

fn vfmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 206077368, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 221, 150, 186, 4, 149, 184, 125, 72, 12], OperandSize::Qword)
}

fn vfmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 186, 196], OperandSize::Dword)
}

fn vfmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 186, 60, 114], OperandSize::Dword)
}

fn vfmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1909673529, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 189, 186, 156, 72, 57, 78, 211, 113], OperandSize::Dword)
}

fn vfmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 149, 165, 186, 206], OperandSize::Qword)
}

fn vfmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDX, 1262857748, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 166, 186, 170, 20, 174, 69, 75], OperandSize::Qword)
}

fn vfmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDX, 783937161, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 182, 186, 146, 137, 238, 185, 46], OperandSize::Qword)
}

fn vfmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 223, 186, 204], OperandSize::Dword)
}

fn vfmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1281071977, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 186, 60, 213, 105, 155, 91, 76], OperandSize::Dword)
}

fn vfmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1795315247, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 222, 186, 44, 197, 47, 86, 2, 107], OperandSize::Dword)
}

fn vfmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 157, 249, 186, 241], OperandSize::Qword)
}

fn vfmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1127251586, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 181, 206, 186, 156, 178, 130, 126, 48, 67], OperandSize::Qword)
}

fn vfmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1557702950, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 219, 186, 140, 134, 38, 169, 216, 92], OperandSize::Qword)
}

