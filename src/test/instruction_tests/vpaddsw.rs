use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 237, 228], OperandSize::Dword)
}

fn vpaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 237, 25], OperandSize::Dword)
}

fn vpaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 237, 217], OperandSize::Qword)
}

fn vpaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 343448782, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 237, 180, 211, 206, 156, 120, 20], OperandSize::Qword)
}

fn vpaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 237, 209], OperandSize::Dword)
}

fn vpaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1165451540, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 237, 4, 205, 20, 97, 119, 69], OperandSize::Dword)
}

fn vpaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 237, 236], OperandSize::Qword)
}

fn vpaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 551544722, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 237, 142, 146, 231, 223, 32], OperandSize::Qword)
}

fn vpaddsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 237, 201], OperandSize::Dword)
}

fn vpaddsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1213414409, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 237, 148, 219, 9, 60, 83, 72], OperandSize::Dword)
}

fn vpaddsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 29, 143, 237, 201], OperandSize::Qword)
}

fn vpaddsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1923903124, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 133, 237, 52, 77, 148, 110, 172, 114], OperandSize::Qword)
}

fn vpaddsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 237, 215], OperandSize::Dword)
}

fn vpaddsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1098722900, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 237, 36, 157, 84, 46, 125, 65], OperandSize::Dword)
}

fn vpaddsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 125, 173, 237, 214], OperandSize::Qword)
}

fn vpaddsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 237, 59], OperandSize::Qword)
}

fn vpaddsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 237, 252], OperandSize::Dword)
}

fn vpaddsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 235307024, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 207, 237, 145, 16, 128, 6, 14], OperandSize::Dword)
}

fn vpaddsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 5, 204, 237, 232], OperandSize::Qword)
}

fn vpaddsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 45, 203, 237, 1], OperandSize::Qword)
}

