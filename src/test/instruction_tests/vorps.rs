use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 86, 247], OperandSize::Dword)
}

fn vorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 361421696, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 156, 88, 128, 219, 138, 21], OperandSize::Dword)
}

fn vorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 86, 196], OperandSize::Qword)
}

fn vorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 86, 30], OperandSize::Qword)
}

fn vorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 86, 237], OperandSize::Dword)
}

fn vorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1546944687, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 86, 180, 179, 175, 128, 52, 92], OperandSize::Dword)
}

fn vorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 86, 207], OperandSize::Qword)
}

fn vorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1492644173, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 86, 12, 221, 77, 241, 247, 88], OperandSize::Qword)
}

fn vorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 139, 86, 197], OperandSize::Dword)
}

fn vorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1133805121, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 141, 86, 188, 176, 65, 126, 148, 67], OperandSize::Dword)
}

fn vorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1356662365, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 159, 86, 164, 207, 93, 6, 221, 80], OperandSize::Dword)
}

fn vorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 76, 139, 86, 249], OperandSize::Qword)
}

fn vorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1875118723, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 36, 137, 86, 164, 86, 131, 10, 196, 111], OperandSize::Qword)
}

fn vorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2144124622, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 36, 154, 86, 28, 205, 206, 190, 204, 127], OperandSize::Qword)
}

fn vorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 174, 86, 254], OperandSize::Dword)
}

fn vorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1354494764, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 170, 86, 156, 114, 44, 243, 187, 80], OperandSize::Dword)
}

fn vorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1785263405, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 189, 86, 134, 45, 245, 104, 106], OperandSize::Dword)
}

fn vorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 36, 169, 86, 225], OperandSize::Qword)
}

fn vorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 4, 170, 86, 4, 83], OperandSize::Qword)
}

fn vorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 76, 188, 86, 12, 192], OperandSize::Qword)
}

fn vorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 206, 86, 199], OperandSize::Dword)
}

fn vorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 203, 86, 58], OperandSize::Dword)
}

fn vorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1054342030, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 222, 86, 4, 189, 142, 251, 215, 62], OperandSize::Dword)
}

fn vorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 100, 204, 86, 219], OperandSize::Qword)
}

fn vorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 12, 197, 86, 57], OperandSize::Qword)
}

fn vorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1516058794, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 20, 215, 86, 132, 144, 170, 56, 93, 90], OperandSize::Qword)
}

