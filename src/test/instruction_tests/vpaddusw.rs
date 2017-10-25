use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 219], OperandSize::Dword)
}

fn vpaddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 567198277, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 221, 138, 69, 194, 206, 33], OperandSize::Dword)
}

fn vpaddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 203], OperandSize::Qword)
}

fn vpaddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 177985873, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 221, 12, 149, 81, 217, 155, 10], OperandSize::Qword)
}

fn vpaddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 221, 236], OperandSize::Dword)
}

fn vpaddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 221, 4, 118], OperandSize::Dword)
}

fn vpaddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 221, 192], OperandSize::Qword)
}

fn vpaddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 221, 18], OperandSize::Qword)
}

fn vpaddusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 221, 228], OperandSize::Dword)
}

fn vpaddusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1273836674, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 221, 36, 85, 130, 52, 237, 75], OperandSize::Dword)
}

fn vpaddusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 29, 137, 221, 225], OperandSize::Qword)
}

fn vpaddusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RBX, 498642927, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 132, 221, 171, 239, 175, 184, 29], OperandSize::Qword)
}

fn vpaddusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 171, 221, 193], OperandSize::Dword)
}

fn vpaddusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1684409958, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 221, 164, 66, 102, 14, 102, 100], OperandSize::Dword)
}

fn vpaddusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 93, 166, 221, 220], OperandSize::Qword)
}

fn vpaddusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1156238326, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 93, 167, 221, 44, 149, 246, 203, 234, 68], OperandSize::Qword)
}

fn vpaddusw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 221, 212], OperandSize::Dword)
}

fn vpaddusw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 221, 44, 153], OperandSize::Dword)
}

fn vpaddusw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 202, 221, 217], OperandSize::Qword)
}

fn vpaddusw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2123415441, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 21, 193, 221, 156, 241, 145, 191, 144, 126], OperandSize::Qword)
}

