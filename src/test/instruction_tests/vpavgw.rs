use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 227, 247], OperandSize::Dword)
}

fn vpavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 62512117, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 227, 177, 245, 219, 185, 3], OperandSize::Dword)
}

fn vpavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 227, 196], OperandSize::Qword)
}

fn vpavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 708274337, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 227, 129, 161, 104, 55, 42], OperandSize::Qword)
}

fn vpavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 227, 239], OperandSize::Dword)
}

fn vpavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 861339805, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 227, 175, 157, 0, 87, 51], OperandSize::Dword)
}

fn vpavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 227, 242], OperandSize::Qword)
}

fn vpavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 440157072, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 227, 28, 197, 144, 67, 60, 26], OperandSize::Qword)
}

fn vpavgw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 227, 197], OperandSize::Dword)
}

fn vpavgw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1270838750, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 227, 188, 127, 222, 117, 191, 75], OperandSize::Dword)
}

fn vpavgw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 85, 142, 227, 243], OperandSize::Qword)
}

fn vpavgw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 604619492, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 37, 130, 227, 36, 77, 228, 194, 9, 36], OperandSize::Qword)
}

fn vpavgw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 227, 205], OperandSize::Dword)
}

fn vpavgw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 1792135827, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 227, 155, 147, 210, 209, 106], OperandSize::Dword)
}

fn vpavgw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 61, 163, 227, 235], OperandSize::Qword)
}

fn vpavgw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 251805077, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 29, 166, 227, 188, 211, 149, 61, 2, 15], OperandSize::Qword)
}

fn vpavgw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 227, 224], OperandSize::Dword)
}

fn vpavgw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 477800182, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 227, 28, 69, 246, 166, 122, 28], OperandSize::Dword)
}

fn vpavgw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 61, 196, 227, 243], OperandSize::Qword)
}

fn vpavgw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 319121570, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 45, 193, 227, 180, 66, 162, 104, 5, 19], OperandSize::Qword)
}

