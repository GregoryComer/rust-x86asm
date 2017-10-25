use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 119, 202], OperandSize::Dword)
}

fn vpermi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2106895435, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 119, 52, 181, 75, 172, 148, 125], OperandSize::Dword)
}

fn vpermi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 93986170, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 119, 4, 133, 122, 29, 154, 5], OperandSize::Dword)
}

fn vpermi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 69, 143, 119, 209], OperandSize::Qword)
}

fn vpermi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 37, 138, 119, 44, 185], OperandSize::Qword)
}

fn vpermi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 93, 158, 119, 58], OperandSize::Qword)
}

fn vpermi2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 119, 193], OperandSize::Dword)
}

fn vpermi2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 119, 15], OperandSize::Dword)
}

fn vpermi2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1463001700, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 191, 119, 20, 205, 100, 162, 51, 87], OperandSize::Dword)
}

fn vpermi2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 69, 172, 119, 254], OperandSize::Qword)
}

fn vpermi2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 416899448, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 5, 167, 119, 180, 95, 120, 97, 217, 24], OperandSize::Qword)
}

fn vpermi2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 29, 178, 119, 20, 214], OperandSize::Qword)
}

fn vpermi2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 119, 242], OperandSize::Dword)
}

fn vpermi2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 119, 51], OperandSize::Dword)
}

fn vpermi2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1001743707, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 119, 164, 114, 91, 101, 181, 59], OperandSize::Dword)
}

fn vpermi2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 37, 198, 119, 216], OperandSize::Qword)
}

fn vpermi2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 77, 201, 119, 60, 128], OperandSize::Qword)
}

fn vpermi2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2026908202, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 61, 214, 119, 12, 77, 42, 42, 208, 120], OperandSize::Qword)
}

