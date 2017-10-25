use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 90, 223], OperandSize::Dword)
}

fn vcvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1366032026, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 90, 36, 197, 154, 254, 107, 81], OperandSize::Dword)
}

fn vcvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 90, 218], OperandSize::Qword)
}

fn vcvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 2042880651, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 90, 148, 242, 139, 226, 195, 121], OperandSize::Qword)
}

fn vcvtsd2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 231, 217, 90, 233], OperandSize::Dword)
}

fn vcvtsd2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 239, 137, 90, 1], OperandSize::Dword)
}

fn vcvtsd2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 199, 251, 90, 232], OperandSize::Qword)
}

fn vcvtsd2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 231, 139, 90, 24], OperandSize::Qword)
}

