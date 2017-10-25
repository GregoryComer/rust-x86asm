use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestnmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 102, 10, 39, 222], OperandSize::Dword)
}

fn vptestnmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 176792386, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 110, 11, 39, 180, 186, 66, 163, 137, 10], OperandSize::Dword)
}

fn vptestnmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 94, 30, 39, 12, 128], OperandSize::Dword)
}

fn vptestnmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 30, 11, 39, 238], OperandSize::Qword)
}

fn vptestnmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 70, 15, 39, 60, 187], OperandSize::Qword)
}

fn vptestnmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1293514787, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 54, 22, 39, 140, 66, 35, 120, 25, 77], OperandSize::Qword)
}

fn vptestnmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 70, 46, 39, 223], OperandSize::Dword)
}

fn vptestnmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1022760993, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 110, 43, 39, 28, 85, 33, 24, 246, 60], OperandSize::Dword)
}

fn vptestnmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 118, 61, 39, 14], OperandSize::Dword)
}

fn vptestnmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 126, 45, 39, 221], OperandSize::Qword)
}

fn vptestnmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 70, 41, 39, 51], OperandSize::Qword)
}

fn vptestnmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 38, 51, 39, 60, 184], OperandSize::Qword)
}

fn vptestnmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 110, 76, 39, 254], OperandSize::Dword)
}

fn vptestnmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 78, 74, 39, 12, 155], OperandSize::Dword)
}

fn vptestnmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 875899643, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 126, 90, 39, 188, 250, 251, 42, 53, 52], OperandSize::Dword)
}

fn vptestnmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 94, 78, 39, 251], OperandSize::Qword)
}

fn vptestnmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RSI, 1849180548, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 46, 66, 39, 182, 132, 65, 56, 110], OperandSize::Qword)
}

fn vptestnmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RCX, 2027256687, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 38, 91, 39, 177, 111, 123, 213, 120], OperandSize::Qword)
}

