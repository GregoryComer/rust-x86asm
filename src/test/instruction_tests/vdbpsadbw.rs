use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdbpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 139, 66, 223, 74], OperandSize::Dword)
}

fn vdbpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 714042752, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 117, 141, 66, 171, 128, 109, 143, 42, 118], OperandSize::Dword)
}

fn vdbpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 101, 137, 66, 238, 18], OperandSize::Qword)
}

fn vdbpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 135, 66, 36, 182, 106], OperandSize::Qword)
}

fn vdbpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 66, 231, 98], OperandSize::Dword)
}

fn vdbpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 567558777, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 171, 66, 132, 112, 121, 66, 212, 33, 91], OperandSize::Dword)
}

fn vdbpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 13, 175, 66, 235, 97], OperandSize::Qword)
}

fn vdbpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 181407483, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 45, 165, 66, 148, 178, 251, 14, 208, 10, 110], OperandSize::Qword)
}

fn vdbpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 204, 66, 213, 114], OperandSize::Dword)
}

fn vdbpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 324718293, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 201, 66, 153, 213, 206, 90, 19, 97], OperandSize::Dword)
}

fn vdbpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 195, 77, 193, 66, 226, 72], OperandSize::Qword)
}

fn vdbpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 61, 193, 66, 16, 115], OperandSize::Qword)
}

