use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 54, 238], OperandSize::Dword)
}

fn vpermd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 54, 39], OperandSize::Dword)
}

fn vpermd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 54, 246], OperandSize::Qword)
}

fn vpermd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 54, 52, 209], OperandSize::Qword)
}

fn vpermd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 54, 223], OperandSize::Dword)
}

fn vpermd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 161129787, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 54, 156, 64, 59, 165, 154, 9], OperandSize::Dword)
}

fn vpermd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 54, 60, 123], OperandSize::Dword)
}

fn vpermd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 61, 171, 54, 209], OperandSize::Qword)
}

fn vpermd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 5, 166, 54, 4, 183], OperandSize::Qword)
}

fn vpermd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RDI, 1742296455, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 29, 189, 54, 167, 135, 85, 217, 103], OperandSize::Qword)
}

fn vpermd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 54, 204], OperandSize::Dword)
}

fn vpermd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 372645916, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 54, 44, 157, 28, 32, 54, 22], OperandSize::Dword)
}

fn vpermd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 220, 54, 52, 246], OperandSize::Dword)
}

fn vpermd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 29, 194, 54, 221], OperandSize::Qword)
}

fn vpermd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RDI, 1493481564, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 21, 199, 54, 191, 92, 184, 4, 89], OperandSize::Qword)
}

fn vpermd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1994349792, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 93, 215, 54, 44, 125, 224, 92, 223, 118], OperandSize::Qword)
}

