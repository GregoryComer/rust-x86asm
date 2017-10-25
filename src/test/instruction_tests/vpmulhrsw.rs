use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 11, 240], OperandSize::Dword)
}

fn vpmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 11, 28, 210], OperandSize::Dword)
}

fn vpmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 11, 221], OperandSize::Qword)
}

fn vpmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 11, 51], OperandSize::Qword)
}

fn vpmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 11, 196], OperandSize::Dword)
}

fn vpmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 11, 56], OperandSize::Dword)
}

fn vpmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 11, 234], OperandSize::Qword)
}

fn vpmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 11, 34], OperandSize::Qword)
}

fn vpmulhrsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 11, 255], OperandSize::Dword)
}

fn vpmulhrsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1958934018, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 11, 132, 219, 2, 246, 194, 116], OperandSize::Dword)
}

fn vpmulhrsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 5, 139, 11, 213], OperandSize::Qword)
}

fn vpmulhrsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 77, 143, 11, 62], OperandSize::Qword)
}

fn vpmulhrsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 11, 218], OperandSize::Dword)
}

fn vpmulhrsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 309633712, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 11, 128, 176, 162, 116, 18], OperandSize::Dword)
}

fn vpmulhrsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 45, 167, 11, 246], OperandSize::Qword)
}

fn vpmulhrsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 166, 11, 7], OperandSize::Qword)
}

fn vpmulhrsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 11, 244], OperandSize::Dword)
}

fn vpmulhrsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDX, 2036965253, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 11, 170, 133, 159, 105, 121], OperandSize::Dword)
}

fn vpmulhrsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 61, 194, 11, 230], OperandSize::Qword)
}

fn vpmulhrsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RDX, 636172249, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 21, 194, 11, 130, 217, 55, 235, 37], OperandSize::Qword)
}

