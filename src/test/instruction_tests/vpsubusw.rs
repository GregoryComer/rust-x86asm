use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 217, 216], OperandSize::Dword)
}

fn vpsubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1491655910, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 217, 188, 187, 230, 220, 232, 88], OperandSize::Dword)
}

fn vpsubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 217, 244], OperandSize::Qword)
}

fn vpsubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 217, 20, 70], OperandSize::Qword)
}

fn vpsubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 217, 223], OperandSize::Dword)
}

fn vpsubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 751270554, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 217, 36, 245, 154, 122, 199, 44], OperandSize::Dword)
}

fn vpsubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 217, 236], OperandSize::Qword)
}

fn vpsubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1822257842, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 217, 188, 201, 178, 114, 157, 108], OperandSize::Qword)
}

fn vpsubusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 217, 201], OperandSize::Dword)
}

fn vpsubusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1323905120, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 217, 132, 139, 96, 48, 233, 78], OperandSize::Dword)
}

fn vpsubusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 53, 141, 217, 203], OperandSize::Qword)
}

fn vpsubusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 69, 130, 217, 8], OperandSize::Qword)
}

fn vpsubusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 217, 199], OperandSize::Dword)
}

fn vpsubusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 217, 36, 240], OperandSize::Dword)
}

fn vpsubusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 5, 171, 217, 252], OperandSize::Qword)
}

fn vpsubusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1959800130, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 162, 217, 36, 93, 66, 45, 208, 116], OperandSize::Qword)
}

