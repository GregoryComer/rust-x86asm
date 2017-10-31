use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 255], OperandSize::Dword)
}

#[test]
fn vpaddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1143245654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 221, 28, 69, 86, 139, 36, 68], OperandSize::Dword)
}

#[test]
fn vpaddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 221, 252], OperandSize::Qword)
}

#[test]
fn vpaddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1880908964, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 221, 52, 213, 164, 100, 28, 112], OperandSize::Qword)
}

#[test]
fn vpaddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 221, 221], OperandSize::Dword)
}

#[test]
fn vpaddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1257908589, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 221, 44, 253, 109, 41, 250, 74], OperandSize::Dword)
}

#[test]
fn vpaddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 221, 193], OperandSize::Qword)
}

#[test]
fn vpaddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1288997155, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 221, 36, 213, 35, 137, 212, 76], OperandSize::Qword)
}

#[test]
fn vpaddusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 221, 238], OperandSize::Dword)
}

#[test]
fn vpaddusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 92669374, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 138, 221, 132, 89, 190, 5, 134, 5], OperandSize::Dword)
}

#[test]
fn vpaddusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 93, 143, 221, 207], OperandSize::Qword)
}

#[test]
fn vpaddusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 5, 138, 221, 3], OperandSize::Qword)
}

#[test]
fn vpaddusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 221, 248], OperandSize::Dword)
}

#[test]
fn vpaddusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1062397742, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 221, 156, 74, 46, 231, 82, 63], OperandSize::Dword)
}

#[test]
fn vpaddusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 117, 173, 221, 250], OperandSize::Qword)
}

#[test]
fn vpaddusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 101, 173, 221, 33], OperandSize::Qword)
}

#[test]
fn vpaddusw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 203, 221, 211], OperandSize::Dword)
}

#[test]
fn vpaddusw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 128165095, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 221, 52, 221, 231, 164, 163, 7], OperandSize::Dword)
}

#[test]
fn vpaddusw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 101, 193, 221, 241], OperandSize::Qword)
}

#[test]
fn vpaddusw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 170906159, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 206, 221, 12, 117, 47, 210, 47, 10], OperandSize::Qword)
}

